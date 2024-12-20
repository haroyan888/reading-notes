import { Card } from "react-bootstrap";

import Book from "~/types/book";
import BookDetailModal from "~/components/Modals/BookDetailModal/BookDetailModal";
import { useState } from "react";

interface props {
	book: Book,
	handleAfterDelete: () => void,
}

export default function BookCard({ ...props }: props) {
	const title_len_max = 30;
	const description_len_max = 40;

	const [book, setBook] = useState<Book>(props.book);
	const [show, setShow] = useState<boolean>(false);
	const handleOpen = () => setShow(true);
	const handleClose = () => setShow(false);

	const onClick = () => handleOpen();
	return (
		<>
			<button onClick={onClick} style={{ margin: "5px" }}>
				<Card>
					<Card.Body className="flex gap-5 w-[30rem] h-40" style={{ backgroundColor: book.is_complete ? "palegreen" : "" }}>
						<Card.Img variant="top" src={book.image_url} className="w-20" />
						<div className="content-center">
							<Card.Title>
								{book.title.length <= title_len_max
									? book.title
									: book.title.slice(0, title_len_max) + '...'}
							</Card.Title>
							<Card.Subtitle className="mb-2 text-muted">
								{book.authors.length == 1
									? book.authors[0]
									: book.authors[0] + ' ...'}
							</Card.Subtitle>
							<Card.Text className="my-auto">
								{book.description.length <= description_len_max
									? book.description
									: book.description.slice(0, description_len_max) + ' ...'}
							</Card.Text>
						</div>
					</Card.Body>
				</Card>
			</button>
			<BookDetailModal book={book} setBook={setBook} show={show} handleClose={handleClose} handleAfterDelete={props.handleAfterDelete} />
		</>
	)
}