import {useState, useEffect}from 'react'
import axios from 'axios';
import { Link } from 'react-router-dom';

const Books = () => {

  const [books, setBooks] = useState([]);

  useEffect(() => {
    //make call to the server
    const fetchAllBooks = async () => {
      try{
        const res = await axios.get("http://localhost:8800/books")
        setBooks(res.data)
      }catch(err){
        console.log(err);
      }
    }

    fetchAllBooks()
  }, [])
  

  const handleDelete = async (id) => {
    try {
      await axios.delete(`http://localhost:8800/books/${id}`)
      window.location.reload()
    } catch (error) {
      console.log(error);
    }
  }

  return (
	<div className='book'>
    <h1 className='book-title'>paper shop</h1>
    {books.map(book => (
      <div key={book.id} className='book-list'>
        {book.cover && <img src={book.cover} alt={book.title} />}
        <h2 className='book-list-title'>{book.title}</h2>
        <p className='book-list-desc'>{book.desc}</p>
        <span className='book-list-price'>{book.price}</span>
        <button className='delete' onClick={() => handleDelete(book.id)}>delete</button>
        <button className='update'><Link to={`/update/${book.id}`}>update</Link></button>
      </div>
      
    ))}
      <button className='book-list-button'><Link to="/add" className='link'>Add new book</Link></button>
  </div>
  )
}

export default Books