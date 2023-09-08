const { users, expenses, salaries } = require("./data");
const { ifs, actions } = require("./fns");

function v1() {
    const isRun = false;

    const tables = {
        marriedTable: [],
        canDrinkTable: [],
        isRichTable: [],
    }

    ifs.forEach((thisif) => {
        users.forEach((user) => {
            const fn_ = thisif['fn'];
            const res = fn_(user[thisif.name], thisif.value);
            if (res) {
                tables[thisif.linkedTable].push(user);
            }
        })
    })

    console.log(tables)
}

function run(left, right) {
    const balances = [];

    actions.forEach((action) => {
        const fn_ = action.fn;
        const doThen = action.doThen;

        const validation = fn_(left[action.left], right[action.right]);
        if (validation) {
            const x = (b) => { balances.push({ id: 1, balance: b, userId: 1 }) }
            doThen(left[action.left], right[action.right], x);
        }
    })

    console.log(balances);
}

run(expenses[0], salaries[0])