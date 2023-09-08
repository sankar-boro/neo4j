const users = [
    {userId: 1, fname: 'Sankar', lname: 'Boro', age: 15, married: false, },
    {userId: 2, fname: 'Amrit', lname: 'Boro', age: 16, married: false }
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
        fn: equalsTo,
        value: true,
    },
    {
        name: 'age',
        type: 'greaterThen',
        value: 18,
        fn: greaterThen
    },
];

module.exports = {
    users,
    ifs
}