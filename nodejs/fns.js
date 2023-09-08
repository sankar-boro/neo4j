const ifs = [
    {
        name: 'married',
        type: 'equalsTo',
        value: true,
        linkedTable: 'marriedTable',
        fn: equalsTo,
    },
    {
        name: 'age',
        type: 'greaterThen',
        value: 18,
        linkedTable: 'canDrinkTable',
        fn: greaterThen,
    },
];

const actions = [
    {
        name: 'salaryDue',
        type: 'notEqualTo',
        left: 'amount',
        right: 'paid',
        fn: notEqualsTo,
        doThen: (left, right, fn__) => {
            const x = left - right
            fn__(x)
        }
    }
]

function notEqualsTo(left, right) {
    if (left !== right) {
        return true;
    }
    return false;
}

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

module.exports = {
    ifs,
    actions,
}