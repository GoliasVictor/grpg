import { type RouteConfig, index, layout, route } from "@react-router/dev/routes";

export default [
  layout("layouts/sidebar.tsx", [
    index("routes/home.tsx"),
    route("table/:id", "routes/table.tsx"),
    route("nodes/", "routes/nodes.tsx")
  ]),
] satisfies RouteConfig;
