import { Link } from "react-router-dom";

import "./index.css";
import { Article } from "../../types/article";
import ArticleCard from "../../components/ArticleCard";

type Props = {
	articles: Article[];
};

export default function TimeLine({ articles }: Props) {
	return (
		<>
			{articles
				.sort((a, b) => (a.post_date > b.post_date ? -1 : 1))
				.map((article) => {
					return (
						<Link to={"/article/" + article.id} key={article.id}>
							<ArticleCard article={article} />
						</Link>
					);
				})}
		</>
	);
}
