import { type RouteConfig, index, route } from "@react-router/dev/routes";

export default [
	index("routes/home.tsx"),
	route("nodes/:id", "routes/node.tsx")
] satisfies RouteConfig;
