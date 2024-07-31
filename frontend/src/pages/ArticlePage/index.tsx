import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import ArticleCard from "../../components/ArticleCard";
import { Article } from "../../types/article";
import getArticle from "../../function/fetchArticle/getArticle";

export default function ArticlePage() {
	const { articleId } = useParams();
	const [article, setArticles] = useState<Article>();

	useEffect(() => {
		if (articleId === undefined) {
			alert("IDが指定されていません。");
			return;
		}
		getArticle(articleId).then((value: Article | undefined) =>
			setArticles(value)
		);
	}, [articleId]);

	return article != undefined ? <ArticleCard article={article} /> : undefined;
}
