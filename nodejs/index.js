const { users, ifs } = require("./data");

function run() {
    const isRun = false;

    ifs.forEach((thisif) => {
        users.forEach((user) => {
            const fn_ = thisif['fn'];
            const res = fn_(user[thisif.name], thisif.value);
            console.log(res);
        })
    })
}

run()