import { Article } from "../../types/article";

type props = {
	article: Article;
	key: string | undefined;
};

export default function ArticleCard({ article, key }: props) {
	return (
		<div className="article" key={key}>
			<p className="article-body">{article.body}</p>
			<div className="article-date">{article.post_date.toString()}</div>
		</div>
	);
}
