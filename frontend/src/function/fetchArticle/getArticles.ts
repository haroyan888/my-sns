import { BASE_SERVER_URL } from "./config";
import { Article } from "../../types/article";

export default async function getArticles() : Promise<Article[]> {
	const res = await fetch(BASE_SERVER_URL + "/article");
	if (!res.ok) {
		alert("記事の取得に失敗しました");
		return [];
	}
	return await res.json();
}