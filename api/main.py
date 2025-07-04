from typing import Literal, Union

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
    direction: GraphDirection

class Node(BaseModel):
    node_id: int
    label: Union[str, None]
class Out(BaseModel):
    item_id: str
    q: str

@app.get("/scalar", include_in_schema=False)
async def scalar_html():
    return get_scalar_api_reference(
        openapi_url=app.openapi_url,
        title=app.title,
    )

@app.post("/node", tags=["node"])
def post_node(label : str) -> NodeResponse:
    id = conn.execute(
        """CREATE (n:Node {label : $label})
        RETURN n.id;""",
        parameters={"label": label}
    ).get_next()[0]

    return NodeResponse(node_id = id)
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

@app.get("/node", tags=["node"])
def get_node() -> list[Node]:
    nodes = conn.execute("MATCH (n:Node) RETURN n.id AS id, n.label as label;").get_as_df()
    print(nodes)
    return [Node(node_id = node["id"], label = node["label"] if isinstance(node["label"], str) else "") for index, node in nodes.iterrows()]

@app.post("/table", tags=["node"])
def table(filter: Filter) -> list[Node]:
    params = {}
    if filter.predicate == None:
        triple_str = "[t:Triple]"
    else:
        triple_str = "[t:Triple {id: $pid} ]"
        params["pid"] = filter.predicate
    if filter.direction == "out":
        rel_str = f"-{triple_str}->"
    else:
        rel_str = f"<-{triple_str}-"
    print(f"""MATCH (n:Node) {rel_str} () RETURN DISTINCT n.id AS id, n.label as label;""")
    nodes = conn.execute(f"""MATCH (n:Node) {rel_str} () RETURN DISTINCT n.id AS id, n.label as label;""", params).get_as_df()
    print(nodes)
    print(help(nodes))

    return [Node(node_id = node["id"], label = node["label"] if not node["label"].isna() else "") for index, node in nodes.iterrows()]

def common_parameters():
    return "carlos"
@app.get("/items/{item_id}")
def read_item(item_id: str, db : Annotated[dict, Depends(common_parameters)], q: Union[str, None] = None) -> Out:
    return Out(item_id=item_id, q=db)