

function test() {
    //const h2 = document.createElement('h2'); console.log(h2)
    //h2.innerHTML = "ASDajfuegvis"
    //document.getElementById("js-container").appendChild(h2);
    const names = ['a', 'b', 'c'];

    const ul = document.querySelector("#js-container > ul");
    for (let i = 0; i < names.length; i ++) {
        const li = document.createElement("li");
        li.setAttribute("class", "list-group-item");
        li.innerHTML = names[i];
        ul.appendChild(li);
    }
}
document.addEventListener("DOMContentLoaded", test)
