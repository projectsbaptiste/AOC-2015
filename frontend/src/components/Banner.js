import React, { useEffect,useState}  from 'react';
// import axios from 'axios';
import Card from 'react-bootstrap/Card';

const Banner = () => {

  // Initialise the state with an empty array
  const [data, setData] = useState([]);


  useEffect(() => {
    async function getNumbers() {
      try {
        const res = await fetch('http://localhost:8080/V1.0/day/1/part/2');
        const data = await res.json();

        // Set the new component state using the data
        setData(data);
      } catch (err) {
        console.log(err);
      }
    }
    getNumbers();
  }, []);

  return (

    <div >
    {data.map((day,_) =>(
              <Card
              bg='light'
              key='Light'
              text='dark'
              style={{ width: '40rem' }}
              border="primary" 
              className="mb-2"
            >
              <Card.Header>Jour  {JSON.stringify(day.day_number)}</Card.Header>
              <Card.Body>
                <Card.Title>Partie {JSON.stringify(day.day_part)} </Card.Title>
                <Card.Text>
                  Resultat {JSON.stringify(day.day_result)}
                </Card.Text>
                <Card.Link href="#">http://localhost:8080/V1.0/day/{JSON.stringify(day.day_number)}/part/{JSON.stringify(day.day_part)}</Card.Link>
              </Card.Body>
            </Card>
     ))}     
</div> 
      );
      
}

/*

<Card.Body>
        <Card.Title>Card Title</Card.Title>
        <Card.Subtitle className="mb-2 text-muted">Card Subtitle</Card.Subtitle>
        <Card.Text>
          Some quick example text to build on the card title and make up the
          bulk of the card's content.
        </Card.Text>
        <Card.Link href="#">Card Link</Card.Link>
        <Card.Link href="#">Another Link</Card.Link>
      </Card.Body>

           {Object.keys(data).map((day,result) =>(
                <div key={JSON.stringify(day)}>
                    <p>{day.day_number}</p>
                </div>
             ))} 
           {Object.entries(data[0]).map(([key, value]) => (
                <div className="item" key={key}>
                    {key} : {JSON.stringify(value)}
                 </div>
             ))} 

        <div className="container">
            {data.map((day,_) =>(
                <div key={JSON.stringify(day)}>
                    <p>{JSON.stringify(day.day_number)}</p>
                    <p>{JSON.stringify(day.day_part)}</p>
                    <p>{JSON.stringify(day.day_result)}</p>
                </div>
             ))}     
        </div> 


  day_number: i32,
    day_part: i32,
    day_result: i32,
////{ {Object.entries(data).map(([key, value]) => (
        <div className="item" key={key}>
          {value}
         </div>
       ))} } const Todo = () => {
*/
//     const [message, setMessage] = useState([]);
//     //var res= "";
//     const apiLink = "http://localhost:8080/V1.0/day/1/part/1";
//{JSON.stringify(data)}

// /*
//     const res =  fetch('http://127.0.0.1:8081/task/1');
//     const data =  res.json();

//     // Set the new component state using the data
//     setMessage(data);*/
    
//     const fetchData = () => {

//         fetch("http://localhost:8080/V1.0/day/1/part/1")
    
//           .then(response => {
//             console.log('TEST1'+response);
//             return response.json()
    
//           })
    
//           .then(data => {
//             console.log('TEST2'+data);
//             setMessage(data);
    
//           })
    
//       }
//       useEffect(() => {

//         fetchData()
    
//       }, [])
    
    

//     /*
//     useEffect(() => {
//         var res= "";
//         var tmp= "";
//         const response =  fetch("http://127.0.0.1:8081/task/1")
//           .then((res) => res.json() )
//           .then(console.log('TEST'+res))
//           .then((res) => setMessage(`Hello with ${res} users`))
//           //.then((res) => console.log(`${res.data} users`))
//           .catch(console.error);
//           tmp = response;
//           console.log(response);
//       }, [setMessage]);
//      */
    
// return (
//    <div>
//       <h1>Todo:</h1>
//       <div>
//           <table>
//           <tr>
//                 {Object.entries(message).map(([key, value]) => (
//                 <th className="item" key={key}>
//                 {key} 
//                 </th>
//             ))}
//             </tr>
//             <tr>
//                 {Object.entries(message).map(([key, value]) => (
//                     <td className="value" key={key}>
//                     {value}
//                     </td>
//                 ))}
//             </tr>
//         </table>
//       </div>
// </div>
// );
// }

export default Banner