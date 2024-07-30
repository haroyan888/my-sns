import { useEffect, useState } from "react";

import "./App.css";
import { Article } from "./types/article";
import ArticleCard from "./components/article/article";

const BASE_SERVER_URL = "http://localhost:8000";

export default function App() {
	const [articles, setArticles] = useState<Article[]>([]);

	const displayArticles = async () => {
		const res = await fetch(BASE_SERVER_URL + "/article");
		if (!res.ok) {
			alert("記事の取得に失敗しました");
			setArticles([]);
		}
		const resJson: Article[] = await res.json();
		setArticles(resJson);
	};

	useEffect(() => {
		displayArticles();
	}, []);

	return (
		<>
			{articles.map((article) => {
				return <ArticleCard article={article} key={article.id} />;
			})}
		</>
	);
}
