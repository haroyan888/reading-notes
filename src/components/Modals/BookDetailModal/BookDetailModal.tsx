import { Button, Modal, Image, Form } from "react-bootstrap";
import ShowMore from "~/components/ShowMore/ShowMore";
import Book from "~/types/book";
import { useEffect, useState, FormEventHandler } from "react";
import { invoke } from "@tauri-apps/api/core";

import ReadingNote from "~/types/memo";
import ReadingNoteList from "~/components/ReadingNoteList/ReadingNoteList";
import ConfirmDialog from "~/components/Modals/ConfirmDialog/ConfirmDialog";

interface props {
	book: Book,
	show: boolean,
	handleClose: () => void,
	handleAfterDelete: () => void,
}

export default function BookDetailModal({ ...props }: props) {

	const [memoList, setReadingNoteList] = useState<ReadingNote[] | undefined>(undefined);
	const [showConfirmDialog, setShowConfirmDialog] = useState<boolean>(false);
	const [memoInput, setMemoInput] = useState("");

	const handleConfirmDialogOpen = () => setShowConfirmDialog(true);
	const handleConfirmDialogClose = () => setShowConfirmDialog(false);

	const getReadingNoteList = () => {
		invoke<ReadingNote[]>("get_reading_notes", { "isbn13": props.book.isbn_13 })
			.then((readingNotes) => setReadingNoteList(readingNotes))
			.catch((e) => alert(e));
	};

	const deleteBook = async () => {
		invoke("delete_book", { "isbn13": props.book.isbn_13 })
			.then(() => {
				handleConfirmDialogClose();
				props.handleClose();
				props.handleAfterDelete();
			})
			.catch((e) => alert(e));
	}

	const handleReadingNoteInputFormSubmit: FormEventHandler<HTMLFormElement> = (event) => {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		const text = form.get("memo-input-form")?.toString();
		invoke("create_reading_note", { text: text, isbn13: props.book.isbn_13 })
			.then(() => {
				setMemoInput("");
				getReadingNoteList()
			})
			.catch((e) => alert(e))
	}

	useEffect(() => {
		getReadingNoteList();
	}, [])

	return (
		<>
			<Modal size="xl" show={props.show} onHide={props.handleClose} centered>
				<Modal.Header closeButton>
					<Modal.Title>詳細</Modal.Title>
				</Modal.Header>

				<Modal.Body className="max-h-[400px] overflow-y-scroll">
					<div className="flex items-top gap-11 flex-wrap justify-center">
						<div className="min-w-[128px]">
							<Image className="w-full" src={props.book.image_url}></Image>
							<Button className="mt-3 w-full" variant="danger" onClick={handleConfirmDialogOpen}>削除</Button>
						</div>
						<div className="max-w-[500px]">
							<h1 className="text-base">タイトル</h1>
							<p className="text-2xl">{props.book.title}</p>
							<p className="text-lg text-right">{props.book.publisher} {props.book.published_date}</p>
							<h1 className="text-base">著者</h1>
							<p className="text-lg">{props.book.authors.join(', ')}</p>
							<h1 className="text-base">説明</h1>
							<ShowMore text={props.book.description} />
							<h1 className="text-base">メモ</h1>
							{memoList?.map((memo) =>
								<ReadingNoteList
									id={memo.id}
									text={memo.text}
									handleAfterDelete={getReadingNoteList}
									key={memo.id} />)}
							<Form onSubmit={handleReadingNoteInputFormSubmit}>
								<Form.Control name="memo-input-form" type="input" value={memoInput} onChange={(e) => setMemoInput(e.target.value)} />
							</Form>
						</div>
					</div>
				</Modal.Body>
			</Modal>
			<ConfirmDialog message={"削除しますか？"} variant="danger" show={showConfirmDialog} handleClose={handleConfirmDialogClose} handleConfirm={deleteBook} />
		</>
	)
}