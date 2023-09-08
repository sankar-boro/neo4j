const users = [
    {userId: 1, fname: 'Sankar', lname: 'Boro', age: 18, married: true, salary: 100000},
    {userId: 2, fname: 'Amrit', lname: 'Boro', age: 16, married: false, salary: 30000 },
    {userId: 2, fname: 'Pankaj', lname: 'Boro', age: 50, married: false, salary: 30000 }
];

const expenses = [
	{
		uid: 1,
		title: "Paid for salary",
		amount: 50000,
	}
]

const salaries = [
	{
        userId: 1,
        expenseId: 1,
        paid: 40000,
    }
]

module.exports = {
    users,
    expenses,
    salaries
}