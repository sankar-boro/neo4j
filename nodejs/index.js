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

function lessThan(left, right) {
    if (left < right) {
        return true
    }
    return false;
}

const comparedFns = {
    lessThan: lessThan
}

function runQuery(mapObj) {
    const reg = /\$[a-zA-Z_]\w*/g
    const str = "INSERT INTO users(userid, fname, lname, email) VALUES($userid, $fname, $lname, $email)";
    const res = str.replaceAll(reg, function(matched){
        return `"${mapObj[matched]}"`;
    });
    console.log(res);
}

function run(row, validation) {
    const fn_ = comparedFns[validation.match];
    let x = fn_(row[validation.matchField], validation.value);
}

runQuery({$userid: 1, $fname: 'Sankar', $lname: 'Boro', $email: 'sankar.boro@yahoo.com' })

// run({
//     id: 1, 
//     paid: 40000, 
//     userId: 1 
// }, 
// { 
//     value: 50000, 
//     match: "lessThan", 
//     matchField: 'paid', 
//     query: "INSERT INTO dueSalaries(due, userId) VALUES($due, $userId)"
// })