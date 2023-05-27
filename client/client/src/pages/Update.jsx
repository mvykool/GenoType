import axios from 'axios';
import {useState} from 'react'
import { useNavigate, useLocation } from 'react-router-dom';

const Update = () => {

  const [newBook, setNewBook] = useState([{
    title: "",
    desc: "",
    price: null,
    cover: ""
  }])

  const navigate = useNavigate();
  const location = useLocation();

  const bookId = location.pathname.split("/")[2];

  const handleChange = (e) => {
    setNewBook(prev => ({...prev, [e.target.name] : e.target.value}))
  };

  const handleClick = async (e) => {
    e.preventDefault()

    try {
      await axios.put("http://localhost:8800/books/" + bookId, newBook)
      navigate("/")
    } catch (error) {
      console.log(error)
    }

  }

  return (
	<div>
    <h1>update the book</h1>
    <input type="text" placeholder='title' onChange={handleChange} name='title' />
    <input type="text" placeholder='desc' onChange={handleChange}  name='desc' />
    <input type="number" placeholder='price' onChange={handleChange}  name='price' />
    <input type="text" placeholder='cover' onChange={handleChange}  name='cover' />
    <button onClick={handleClick} >update</button>
  </div>
  )
}

export default Update