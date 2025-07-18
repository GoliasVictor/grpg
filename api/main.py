from typing import Literal, Optional, Union

from fastapi import FastAPI
from typing import Annotated
from fastapi import Depends, FastAPI
from pydantic import BaseModel, EmailStr
from scalar_fastapi import get_scalar_api_reference
import kuzu
from enum import Enum, IntEnum
import math 
from fastapi.middleware.cors import CORSMiddleware

db = kuzu.Database("./demo_db")
conn = kuzu.Connection(db)
conn.execute(
    """
        CREATE NODE TABLE IF NOT EXISTS Node(id SERIAL, label STRING, PRIMARY KEY (id));
        CREATE NODE TABLE IF NOT EXISTS Predicate(id SERIAL, label STRING, PRIMARY KEY (id));
        CREATE REL TABLE IF NOT EXISTS Triple(FROM Node TO Node, id INT64);
    """
)
origins = [
    "http://localhost:5173",
]
app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

class NodeResponse(BaseModel):
    node_id: int
class GraphDirection(str, Enum):
    in_d = 'in'
    out_d = 'out'

class Filter(BaseModel):
    node_id: Union[int, None]
    predicate: Union[int, None]
    direction: Optional[GraphDirection]

class TableRow(BaseModel):
    node_id: int
    label: Union[str, None]
    pid: Union[int, None]

class Node(BaseModel):
    node_id: int
    label: str
class Out(BaseModel):
    item_id: str
    q: str
class Predicate(BaseModel):
    id: int
    label: str
class PostPredicate(BaseModel):
    label: str
@app.get("/scalar", include_in_schema=False)
async def scalar_html():
    return get_scalar_api_reference(
        openapi_url="http://localhost:8000/openapi.json",

        title=app.title,
    )

@app.post("/node", tags=["node"])
def post_node(label : str) -> NodeResponse:
    last_id = conn.execute("MATCH (n:Node) RETURN MAX(n.id) AS id;").get_next()[0]
    if last_id is None:
        last_id = 0
    id = conn.execute(
        """CREATE (n:Node {id: $id, label : $label})
        RETURN n.id;""",
        parameters={"label": label, "id": last_id + 1}
    ).get_next()[0]

    return NodeResponse(node_id = id)

@app.put("/node/{node_id}", tags=["node"])
def put_node(node_id: int, label: str) -> Node:
    new_label = conn.execute(
        """MATCH (n:Node {id: $id}) SET n.label = $label RETURN n.label;""",
        parameters={"id": node_id, "label": label}
    ).get_next()[0]
    return Node(node_id = node_id, label = new_label)

@app.delete("/node/{node_id}", tags=["node"])
def delete_node(node_id: int) -> NodeResponse:
    conn.execute(
        """MATCH (n:Node {id: $id}) DETACH DELETE n;""",
        parameters={"id": node_id}
    )
    return NodeResponse(node_id = node_id)
class Triple(BaseModel): 
    subject_id: int 
    predicate_id: int
    object_id: int 
    
@app.post("/triple", tags=["triple"])
def post_triple(triple: Triple):
    conn.execute(
        """MATCH (n1:Node), (n2:Node)
        WHERE n1.Id = $id1 AND n2.Id = $id2
        CREATE (n1)-[t:Triple  { id: $pid }]->(n2)
        RETURN t.Id;""",
        parameters={"id1": triple.subject_id, "pid":  triple.predicate_id, "id2":  triple.object_id}
    )
@app.delete("/triple", tags=["triple"])
def delete_triple(triple: Triple):
    conn.execute(
        """MATCH (n1:Node {id : $id1})-[t:Triple {id: $pid}]->(n2:Node {id: $id2})
        DELETE t;""",
        parameters={"id1": triple.subject_id, "pid":  triple.predicate_id, "id2":  triple.object_id}
    )

@app.get("/predicates", tags=["predicates"])
def get_predicates() -> list[Predicate]:
    predicates = conn.execute("MATCH (p:Predicate) RETURN p.id AS id, p.label as label;").get_as_df()
    print(predicates)
    return [
        Predicate(
            id = predicate["id"],
            label = predicate["label"] if isinstance(predicate["label"], str) else "") 
        for _, predicate in predicates.iterrows()]

@app.post("/predicate", tags=["predicates"])
def post_predicate(predicate: PostPredicate) -> Predicate :

    last_id = conn.execute("MATCH (p:Predicate) RETURN MAX(p.id) AS id;").get_next()[0]
    if last_id is None:
        last_id = 0
    
    id = conn.execute(
        """CREATE (p:Predicate {label : $label, id: $id})
        RETURN p.id;""",
        parameters={"label": predicate.label, "id": last_id + 1}
    ).get_next()[0]

    return Predicate(id = id, label = predicate.label)
@app.get("/node", tags=["node"])
def get_node() -> list[Node]:
    nodes = conn.execute("MATCH (n:Node) RETURN n.id AS id, n.label as label;").get_as_df()
    print(nodes)
    return [Node(node_id = node["id"], label = node["label"] if isinstance(node["label"], str) else "") for index, node in nodes.iterrows()]

@app.post("/table", tags=["node"])
def table(filter: Filter) -> list[TableRow]:
    params = {}
    if filter.predicate == None:
        triple_str = "[t:Triple]"
    else:
        triple_str = "[t:Triple {id: $pid} ]"
        params["pid"] = filter.predicate
    if filter.direction == "out":
        rel_str = f"-{triple_str}->"
    elif filter.direction == "in":
        rel_str = f"<-{triple_str}-"
    else:
        rel_str = f"-{triple_str}-"
    node_str = ""
    if filter.node_id is not None:
        node_str = "(:Node { id: $node_id })"
        params["node_id"] = filter.node_id
    else:
        node_str = "(:Node)"
    print(f"""MATCH (n:Node) {rel_str} {node_str} RETURN DISTINCT n.id AS id, n.label as label;""")
    nodes = conn.execute(f"""MATCH (n:Node) {rel_str} {node_str} RETURN DISTINCT n.id AS id,t.id as pid, n.label as label;""", params).get_as_df()
    print(nodes)

    return [
        TableRow(
            node_id = node["id"], 
            label = node["label"] if isinstance(node["label"], str) else "",
            pid = node["pid"]
        ) 
        for index, node in nodes.iterrows()
    ]

example_row_request = {
	"node_id": 1,
	"collumns": [
		{
			"id": 0,
			"filter": {
				"direction": "in",
				"predicate_id": 1, 
			}
		}
	]
}
example_response = {
    "columns": [
        { 
            "id": 0,
            "values": [
                1, 2, 6
            ]
        }
    ]
}
class ColumnFilter(BaseModel):
    direction: Optional[Literal["in", "out"]]
    predicate_id: int 
class ColumnDefinition(BaseModel):
    id: int
    filter: ColumnFilter
class GetTable(BaseModel):
    nodes_id: list[int]
    columns: list[ColumnDefinition]

class CellResponse(BaseModel):
    id: int
    values: list[int]

class RowRespone(BaseModel):
    node_id: int
    columns: list[CellResponse]


@app.post("/full-table", tags=["table"])
def table(parms: GetTable) -> list[RowRespone]:
    result = {"columns": []}
    if len(parms.nodes_id) == 0:
        return []
    query = """
        MATCH (c:Node)-[r]->(a) 
        WHERE r.id in $out_ids and c.id in $nids
        RETURN c.id as rnid, a.id as nid, r.id as pid, "out" as direction
        ORDER BY a.id

        UNION ALL
        MATCH (c:Node)<-[r]-(a)
        WHERE r.id in $in_ids and c.id in $nids
        RETURN c.id as rnid, a.id as nid, r.id as pid, "in" as direction
        ORDER BY a.id

        UNION ALL
        MATCH (c:Node)-[r]-(a)
        WHERE r.id in $any_ids and c.id in $nids
        RETURN c.id as rnid, a.id as nid, r.id as pid, "any" as direction
        ORDER BY a.id;"""
    
    query_params = {
        "nids": parms.nodes_id,
        "out_ids":[
            col.filter.predicate_id for col in parms.columns if col.filter.direction == "out"
        ],
        "in_ids": [
            col.filter.predicate_id for col in parms.columns if col.filter.direction == "in"
        ],
        "any_ids":[
            col.filter.predicate_id for col in parms.columns if col.filter.direction is None
        ],
    }
    res = conn.execute(query, query_params).get_as_df()
    rows = {
        nid: [
            {"id": col.id, "values": [] }
            for col in parms.columns
        ] for nid in parms.nodes_id
    }

    for nid, row in res.groupby("rnid").apply(lambda x: x.to_dict(orient='records')).items():                
        columns_result = [
            {
                "id": col.id, 
                "values": [
                    cell["nid"]
                    for cell in row
                    if  (col.filter.direction is None or cell["direction"] == col.filter.direction) and
                    (col.filter.predicate_id is None or cell["pid"] == col.filter.predicate_id)
                ]
            }
            for col in parms.columns
        ]
        rows[nid] = columns_result

    return [
        RowRespone(
            node_id=node_id,
            columns=[
                CellResponse(id=col["id"], values=col["values"]) for col in columns_result
            ]
        ) for node_id, columns_result in rows.items()
    ]


def common_parameters():
    return "carlos"
@app.get("/items/{item_id}")
def read_item(item_id: str, db : Annotated[dict, Depends(common_parameters)], q: Union[str, None] = None) -> Out:
    return Out(item_id=item_id, q=db)