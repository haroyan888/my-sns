import { BrowserRouter as Router, Routes, Route } from "react-router-dom";

import TimeLine from "./pages/TimeLine";
import ArticlePage from "./pages/ArticlePage";
import "./App.css";

export default function App() {

	return (
		<Router>
			<Routes>
				<Route path="/" Component={TimeLine} />
				<Route path="/article/:articleId" Component={ArticlePage} />
			</Routes>
		</Router>
	);
}
