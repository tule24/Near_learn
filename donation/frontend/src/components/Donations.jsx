import React from 'react'

function Donations({donations}) {
  console.log(donations)
  return (
    <>
      <h4> Latest Donations </h4>
      <table className="table table-striped">
        <thead>
          <tr>
            <th scope="col">User</th>
            <th scope="col">Total Donated Ⓝ</th>
          </tr>
        </thead>
        <tbody id="donations-table">
        {donations?.map((ele, i) => {
            return(
              <tr key={i}>
                <td>{ele.account_id}</td>
                <td>{ele.total_amount}</td>
              </tr>
            ) 
          })}
        </tbody>
      </table>
    </>
  )
}

export default Donations