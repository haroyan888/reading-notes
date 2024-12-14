import {Modal, Button, Form, Spinner} from "react-bootstrap";
import {useState, ChangeEventHandler} from "react";
import { invoke } from "@tauri-apps/api/core";

interface props {
	show: boolean,
	handleClose: () => void,
	afterCreateHandler: () => void,
}

export default function CreateBookModal({show, handleClose, afterCreateHandler}: props) {
	const [isbn13, setIsbn13] = useState<string>("");
	const [enableInput, setEnableInput] = useState<boolean>(true);
	const [enableSubmit, setEnableSubmit] = useState<boolean>(false);
	const [isLoading, setIsLoading] = useState<boolean>(false);
	const onChangeIsbn13Form: ChangeEventHandler<HTMLInputElement> = ({target}) => {
		const inputIsbn = target.value;
		setIsbn13(inputIsbn);
		setEnableSubmit(inputIsbn.length === 13);
	}
	const onSubmit = () => {
		setEnableInput(false);
		setEnableSubmit(false);
		setIsLoading(true);
		invoke("create_book", {isbn13: isbn13})
			.then(() => {
				afterCreateHandler();
				setIsbn13("");
			})
			.catch((e) => alert(e))
			.finally(() => {
				setEnableInput(true);
				setEnableSubmit(true);
				setIsLoading(false);
			});
	}
	return (
		<Modal show={show} onHide={handleClose} centered>
			<Modal.Header closeButton>
				<Modal.Title>本の登録</Modal.Title>
			</Modal.Header>

			<Modal.Body>
				<Form onSubmit={onSubmit}>
					<Form.Group
						onChange={onChangeIsbn13Form}
						className="mb-3"
						controlId="exampleForm.ControlInput1"
					>
						<Form.Label>ISBN 13</Form.Label>
						<Form.Control placeholder="1234567890123" disabled={!enableInput} />
					</Form.Group>
				</Form>
			</Modal.Body>

			<Modal.Footer>
				<Button variant="secondary" onClick={handleClose} className="h-10 w-20">閉じる</Button>
				<Button
					variant="primary"
					disabled={!enableSubmit}
					onClick={onSubmit}
					className="h-10 w-20"
				>
					{!isLoading
						? "登録"
						: <Spinner animation="border"  size="sm"/>
					}
				</Button>
			</Modal.Footer>
		</Modal>
	)
}