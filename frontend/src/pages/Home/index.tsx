import { useState, useEffect } from "react";

import { Article } from "../../types/article";
import getArticles from "../../function/fetchArticle/getArticles";
import TimeLine from "../../features/TimeLine";
import "./index.css";

export default function Home() {
	const [articles, setArticles] = useState<Article[]>([]);

	useEffect(() => {
		getArticles().then((value: Article[]) => setArticles(value));
	}, []);

	return <TimeLine articles={articles} />;
}
