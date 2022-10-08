let recentSearch = "";
console.log("start");
function test() {
    //const h2 = document.createElement('h2'); console.log(h2)
    //h2.innerHTML = "ASDajfuegvis"
    //document.getElementById("js-container").appendChild(h2);
    const names = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

    const ul = document.querySelector("#js-container > ul");
    for (let i = 0; i < names.length; i ++) {
        const li = document.createElement("li");
        li.setAttribute("class", "list-group-item");
        li.innerHTML = names[i];
        ul.appendChild(li);
    }
}

function requestSearch(searchTerm) {
    console.log(searchTerm);
}

function init() {
    test();
    const searchButton = document.getElementById('search-button');
    const searchInput = document.getElementById('search-input');
    searchButton.addEventListener('click', () => {

        const inputValue = searchInput.value;
        if (inputValue !== recentSearch) {
            requestSearch(inputValue);
        }
    });
}

document.addEventListener("DOMContentLoaded", init);