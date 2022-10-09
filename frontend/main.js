let recentSearch = '';
let buttons

console.log('start');
function test() {
  //const h2 = document.createElement('h2'); console.log(h2)
  //h2.innerHTML = "ASDajfuegvis"
  //document.getElementById("js-container").appendChild(h2);
  const names = [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'w',
    'x',
    'y',
    'z',
  ];

  const ul = document.querySelector('#js-container > ul');
  for (let i = 0; i < names.length; i++) {
    const li = document.createElement('li');
    li.setAttribute('class', 'list-group-item');
    li.innerHTML = names[i];
    ul.appendChild(li);
  }
}

function gridBuilder(productsList) {
  const ul = document.querySelector('#mainGrid > ul');
  for (let i  = 0; i < productsList.length; i++) {
    const li = document.createElement("li");
    li.setAttribute("class", "list-group-item");
    const certificate = productsList[i].certificate !== undefined ? productsList[i].certificate : "";
    li.innerHTML = productsList[i].name + " " + productsList[i].origin + " " + certificate;
    ul.appendChild(li);
  };
}

function createCards(response) {
  productsList = response.products;
  const container = document.querySelector("#cardRow");
  for (let i = 0; i < productsList.length; i++) {
    const card = createCard(productsList[i], i);
    container.appendChild(card);
  }
}

function createCard(textObj, position) {
  const card = document.createElement("div");
  card.setAttribute("class", "card smallCard");
  //card.setAttribute("style", "max-width: 15rem; margin: 1rem");
  card.setAttribute("id", "card" + position);
  const cardBody = document.createElement("div");
  cardBody.setAttribute("class", "card-body");
  const cardHeading = document.createElement("h5" );
  cardHeading.setAttribute("class", "card-title");
  cardHeading.setAttribute("id", "title" + position);
  cardHeading.innerText = textObj.name;
  const cardText = document.createElement("p");
  cardText.setAttribute("class", "card-text");
  cardText.setAttribute("id", "text" + position);
  cardText.innerText = textObj.origin;
  const cardButton = document.createElement("button");
  cardButton.setAttribute("class", "btn btn-secondary");
  cardButton.setAttribute("type", "button" );
  cardButton.setAttribute("id", "button" + position);
  cardButton.setAttribute("onclick", "toggleCardExpansion(this.id)");
  cardButton.innerText = "vergrößern";
  cardBody.appendChild(cardHeading);
  cardBody.appendChild(cardText);
  cardBody.appendChild(cardButton);
  card.appendChild(cardBody);
  return card;
}

function createInfoTable(infos){

}

function toggleCardExpansion(buttonid) {
  let index = buttonid.replace("button", "");
  let card = document.getElementById("card"+index);
  if (card.classList.contains("smallCard")){
    console.log("klein");
    card.setAttribute("class", card.className.replace('smallCard', 'bigCard'));
    const additionalInfoContainer = document.createElement("div");
    additionalInfoContainer.setAttribute("class", "card-body");
    additionalInfoContainer.setAttribute("id", "addConf" + index);
    additionalInfoContainer.innerText = "asdasd";
    card.appendChild(additionalInfoContainer);
  }
  else{
    console.log("groß");
    card.setAttribute("class", card.className.replace('bigCard', 'smallCard'));
    card.removeChild(document.getElementById("addConf" + index));
  }
}


function processResponse(response) {
  console.log(response);
  console.log(response.products[0]);
  //gridBuilder(response.products);
  const container = document.querySelector("#cardRow");
  createCards(response);
}

function requestSearch(searchTerm) {
  console.log(searchTerm);
  //const url = 'http://127.0.0.1:8000/products?%a%';
  const url = "test.json";
  fetch(url)
        .then((response) => response.json())
        .then((data) => processResponse(data));

}

function init() {
  const searchButton = document.getElementById('search-button');
  const searchInput = document.getElementById('search-input');
  searchButton.addEventListener('click', () => {
    const inputValue = searchInput.value;
    if (inputValue !== recentSearch) {
      requestSearch(inputValue);
    }
  });
}

document.addEventListener('DOMContentLoaded', init);
