import { BASE_SERVER_URL } from "./config";

export default async function getArticle (articleId: string) {
	const res = await fetch(BASE_SERVER_URL + "/article/" + articleId);
	if (!res.ok) {
		alert("記事の取得に失敗しました");
		return undefined;
	}
	return await res.json();
}