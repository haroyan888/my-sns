import { BASE_SERVER_URL } from "./config"

export default async function deleteArticle(id: string) {
	const res = await fetch(BASE_SERVER_URL + "/article/" + id, {method: "DELETE"});
	if(!res.ok) {
		alert("削除に失敗しました。");
		return;
	}
}