import { useEffect, useState } from "react";
import { useParams } from 'react-router-dom';

import ArticleCard from "../../components/ArticleCard";
import { Article } from "../../types/article";

const BASE_SERVER_URL = import.meta.env.VITE_BASE_SERVER_URL;

const getArticle = async (articleId: string) => {
	const res = await fetch(BASE_SERVER_URL + "/article/" + articleId);
	if (!res.ok) {
		alert("記事の取得に失敗しました");
		return undefined;
	}
	return await res.json();
};

export default function ArticlePage() {
	const { articleId } = useParams();
	const [article, setArticles] = useState<Article>();

	useEffect(() => {
		if(articleId === undefined) {
			alert("IDが指定されていません。");
			return;
		}
		getArticle(articleId).then((value: Article|undefined) => setArticles(value));
	}, [articleId]);

	return article != undefined ? 
		<ArticleCard article={article} /> : undefined
}