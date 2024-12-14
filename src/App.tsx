import { useEffect, useState } from "react";
import { Button } from "react-bootstrap";
import { invoke } from "@tauri-apps/api/core";
import { TiPlus } from "react-icons/ti";

import Book from "~/types/book";
import BookCard from "~/components/BookCard/BookCard";
import CreateBookModal from "~/components/Modals/CreateBookModal/CreateBookModal";

export default function Index() {
	const [books, setBooks] = useState<Book[]>([]);
	const [show, setShow] = useState(false);
	const handleClose = () => setShow(false);
	const handleShow = () => setShow(true);

	const getBooksInfo = async () => {
		invoke<Book[]>("all_book")
			.then((books) => setBooks(books))
			.catch((e) => {
				console.log("all book: ", e);
				setBooks([]);
			});
	};

	const afterCreateHandler = () => {
		handleClose();
		void getBooksInfo();
	};

	useEffect(() => {
		void getBooksInfo();
	}, []);

	return (
		<>
			<header className="w-full p-3 flex justify-end bg-blue-500"></header>
			<div className="font-sans p-4 flex flex-wrap justify-content-center">
				{books.map((book) => (
					<BookCard
						book={book}
						key={book.isbn_13}
						handleAfterDelete={getBooksInfo}
					/>
				))}
			</div>
			<Button
				className="fixed bottom-[12px] right-[12px] w-[3rem] h-[3rem] rounded-full"
				variant="primary"
				onClick={handleShow}
			>
				<TiPlus className="scale-[2.0] m-auto" />
			</Button>
			<CreateBookModal
				show={show}
				handleClose={handleClose}
				afterCreateHandler={afterCreateHandler}
			/>
		</>
	);
}
