import { useState, useEffect, FormEvent } from "react";

import { Article } from "../../types/article";
import getArticles from "../../function/fetchArticle/getArticles";
import TimeLine from "../../features/TimeLine";
import "./index.css";
import createArticle from "../../function/fetchArticle/createArticle";

export default function Home() {
	const [articles, setArticles] = useState<Article[]>([]);

	const onSubmitArticle = (event: FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		const eventTarget = event.target as HTMLFormElement;
		const articleBody = eventTarget["article-body"].value;
		if (
			articleBody === undefined ||
			articleBody === null ||
			articleBody === ""
		) {
			return;
		}
		createArticle(articleBody).then(() => (window.location.href = "/"));
	};

	useEffect(() => {
		getArticles().then((value: Article[]) => setArticles(value));
	}, []);

	return (
		<>
			<form action="" className="post-article" onSubmit={onSubmitArticle}>
				<label htmlFor="post-article_article-body">いまどうしてる？</label>
				<input type="text" name="article-body" id="post-article_article-body" />
				<input type="submit" value="投稿" />
			</form>
			<TimeLine articles={articles} />
		</>
	);
}
