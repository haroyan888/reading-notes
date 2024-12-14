import {Form} from "react-bootstrap";
import { TiDelete } from "react-icons/ti";
import { invoke } from "@tauri-apps/api/core";

interface props {
	text: string,
	id: string,
	handleAfterDelete: () => void,
}

export default function ReadingNoteList({id, text, handleAfterDelete}: props) {
	const handleClick = () => {
		invoke("delete_reading_note", {id: id})
			.then(() => handleAfterDelete())
			.catch((e) => alert(e));
	}
	return(
		<div className="flex gap-4 mt-4 mb-4">
			<Form.Control
				type="input"
				value={text}
				disabled
			/>
			<button className="text-red-500" onClick={handleClick}><TiDelete className="w-8 h-8" /></button>
		</div>
	)
}