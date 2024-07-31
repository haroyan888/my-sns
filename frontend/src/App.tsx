import { BrowserRouter as Router, Routes, Route } from "react-router-dom";

import Home from "./pages/Home";
import ArticlePage from "./pages/ArticlePage";
import "./App.css";

export default function App() {
	return (
		<Router>
			<Routes>
				<Route path="/" Component={Home} />
				<Route path="/article/:articleId" Component={ArticlePage} />
			</Routes>
		</Router>
	);
}
