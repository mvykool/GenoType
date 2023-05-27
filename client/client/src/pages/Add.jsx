import axios from 'axios';
import {useState} from 'react'
import { useNavigate } from 'react-router-dom';

const Add = () => {

  const [newBook, setNewBook] = useState([{
    title: "",
    desc: "",
    price: null,
    cover: ""
  }])

  const navigate = useNavigate();

  const handleChange = (e) => {
    setNewBook(prev => ({...prev, [e.target.name] : e.target.value}))
  };

  const handleClick = async (e) => {
    e.preventDefault()

    try {
      await axios.post("http://localhost:8800/books", newBook)
      navigate("/")
    } catch (error) {
      console.log(error)
    }

  }

  return (
	<div>
    <h1>add new book</h1>
    <input type="text" placeholder='title' onChange={handleChange} name='title' />
    <input type="text" placeholder='desc' onChange={handleChange}  name='desc' />
    <input type="number" placeholder='price' onChange={handleChange}  name='price' />
    <input type="text" placeholder='cover' onChange={handleChange}  name='cover' />
    <button onClick={handleClick} >Add</button>
  </div>
  )
}

export default Add