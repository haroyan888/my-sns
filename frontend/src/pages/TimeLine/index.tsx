import { useEffect, useState } from "react";
import { Link } from "react-router-dom";

import "./index.css";
import { Article } from "../../types/article";
import ArticleCard from "../../components/ArticleCard";

const BASE_SERVER_URL = import.meta.env.VITE_BASE_SERVER_URL;

const getArticles = async () => {
	const res = await fetch(BASE_SERVER_URL + "/article");
	if (!res.ok) {
		alert("記事の取得に失敗しました");
		return [];
	}
	return await res.json();
};

export default function TimeLine() {
	const [articles, setArticles] = useState<Article[]>([]);

	useEffect(() => {
		getArticles().then((value: Article[]) => setArticles(value));
	}, []);

	return (
		<>
			{articles.map((article) => {
				return (
					<Link to={"/article/" + article.id } key={article.id}>
						<ArticleCard article={article}/>
					</Link>
				);
			})}
		</>
	);
}