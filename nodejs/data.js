const users = [
    {userId: 1, fname: 'Sankar', lname: 'Boro', age: 18, married: true, salary: 100000},
    {userId: 2, fname: 'Amrit', lname: 'Boro', age: 16, married: false, salary: 30000 },
    {userId: 2, fname: 'Pankaj', lname: 'Boro', age: 50, married: false, salary: 30000 }
];

function equalsTo(left, right) {
    if (left === right) {
        return true
    }
    return false;
}

function greaterThen(left, right) {
    if (left > right) {
        return true
    }
    return false;
}

const ifs = [
    {
        name: 'married',
        type: 'equalsTo',
        value: true,
        linkedTable: 'marriedTable',
        fn: equalsTo,
        then: () => {},
    },
    {
        name: 'age',
        type: 'greaterThen',
        value: 18,
        linkedTable: 'canDrinkTable',
        fn: greaterThen,
        then: () => {},
    },
];

module.exports = {
    users,
    ifs
}