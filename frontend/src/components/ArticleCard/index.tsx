import deleteArticle from "../../function/fetchArticle/deleteArticle";
import { Article } from "../../types/article";

type props = {
	article: Article;
	key?: string;
};

export default function ArticleCard({ article, key }: props) {
	const onClickDeleteBtn = () => {
		deleteArticle(article.id).then(() => (window.location.href = "/"));
	};

	return (
		<div className="article" key={key}>
			<div className="article-header">
				<div className="article-date">{article.post_date.toString()}</div>
				<button className="article-delete-button" onClick={onClickDeleteBtn}>
					Delete
				</button>
			</div>
			<p className="article-body">{article.body}</p>
		</div>
	);
}
