import { BASE_SERVER_URL } from "./config";

export default async function createArticle(articleBody: string) {
	const res = await fetch(
		BASE_SERVER_URL + "/article", 
		{
			method: "POST", 
			headers: {"CONTENT-TYPE": "application/json"},
			body: JSON.stringify({
				"body": articleBody
			})
		}
	);
	if(!res.ok) {
		alert("投稿に失敗しました。");
		return;
	}
}