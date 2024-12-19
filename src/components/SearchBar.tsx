interface props {
	onChange: (keyword: string) => void
};

function SearchBar({onChange}: props) {
	const changeInput = (e: React.ChangeEvent<HTMLInputElement>) => 
		onChange(e.target.value);

	return <input className="p-2 rounded-md" type="search" placeholder="ここにキーワード" onChange={changeInput} />
}

export default SearchBar;