import config from "./config.js";

console.log("Loaded popup.js");

const homePage = document.getElementById("home-page");
const chatPage = document.getElementById("chat-page");

const messageInput = document.getElementById("message-input");

const messageTemplate = document.getElementById("message-template");
messageTemplate.hidden = true;

let isChatting = localStorage.getItem("isChatting") === "true" ? true : false;

if (isChatting) {
    homePage.hidden = true;
    chatPage.hidden = false;
    connectToRoom();
} else {
    homePage.hidden = false;
    chatPage.hidden = true;
}

function createMessage(name, content) {
    console.log(`Creating message (${name}) (${content})`);

    let message = messageTemplate.cloneNode(true);
    message.id = null;
    message.querySelector("b").textContent = name;
    message.querySelector("p").textContent = content;

    message.hidden = false;
    messageTemplate.parentElement.insertBefore(message, messageTemplate.nextSibling);
}

function connectToRoom() {
    let roomId = localStorage.getItem("roomId");

    console.log("Fetching existing messages");

    document.getElementById("messages-list").querySelectorAll("*:not(#message-template)").forEach((element) => {
        if (element.parentNode.id !== "message-template") { element.remove(); }
    });
    
    fetch(config.BASE_URL + "/api/v1/message/room/" + roomId)
        .then(response => response.json())
        .then(data => {
            console.log(`Found ${data.length} existing messages`, data);

            data.reverse();

            data.forEach((message) => {
                createMessage(message.sender.name, message.content);
            })
        })

    var source = new EventSource(config.BASE_URL + "/api/v1/room/" + localStorage.getItem("roomId") + "/events");
    source.onmessage = (event) => {
        let data = JSON.parse(event.data);

        console.log("Received data from event stream", data);

        createMessage(data.sender.name, data.content);
    }
    document.getElementById("leave-room").addEventListener("click", () => {
        source.close();
    })
}

messageInput.addEventListener("keydown", (event) => {
    if (event.key !== "Enter" || messageInput.value.length < 1) { return; };

    let content = messageInput.value;

    messageInput.value = "";

    console.log(`Attempting to send message (${content})`);

    fetch(config.BASE_URL + "/api/v1/message",
        {
            method: "POST",
            body: JSON.stringify({
                user_id: localStorage.getItem("userId"),
                content: content,
            })
        }
    )
    .then(response => {
        if (response.status === 200) {
            console.log("Successfully sent message", response.data);
        } else { console.log("Failed to send message", response); }
    })
    .catch(error => {
        console.error("An error occurred while trying to send a message", error);
    })
})

document.getElementById("leave-room").addEventListener("click", () => {
    localStorage.setItem("isChatting", String(false));
    localStorage.setItem("roomId", null);
    localStorage.setItem("userId", null);

    homePage.hidden = false;
    chatPage.hidden = true;
})

document.getElementById("join-submit").addEventListener("click", () => {
    let roomInput = document.getElementById("join-room-input");
    let roomId = roomInput.value;

    let userInput = document.getElementById("user-input");
    let username = userInput.value;

    if (username.length < 1) { return; }

    const joinNotice = document.getElementById("join-notice");

    console.log(`Attempting to join room (${roomId})`);

    fetch(config.BASE_URL + "/api/v1/user",
        {
            method: "POST",
            body: JSON.stringify({
                room_id: roomId,
                name: username,
            })
        }
    )
    .then(response => {
        if (response.status === 500) { // code 500 for this is a horrible idea
            joinNotice.textContent = "Room not found";
            throw new Error("No room with given ID was found");
        } else { return response.json(); }
    })
    .then(data => {
        console.log("Successfully joined room", data);

        localStorage.setItem("isChatting", String(true));

        localStorage.setItem("userId", data.user_id);
        localStorage.setItem("roomId", data.room_id);

        homePage.hidden = true;
        chatPage.hidden = false;

        connectToRoom();
    })
    .catch(error => {
        console.error("An error occurred when joining room", error);
    })
})

const createSubmit = document.getElementById("create-submit");

createSubmit.addEventListener("click", () => {
    const input = document.getElementById("create-input");
    let roomId = input.value;

    const createNotice = document.getElementById("create-notice");

    console.log(`Attempting to create room (${roomId})`);

    createSubmit.disabled = true;

    fetch(config.BASE_URL + "/api/v1/room",
        {
            method: "POST",
            body: JSON.stringify({
                "room_id": roomId,
            })
        }
    )
    .then(response => {
        if (response.status === 409) {
            createNotice.textContent = "ID already in use"
            throw new Error("Failed to create room, ID already in use");
        } else {
            return response.json();
        }
    })
    .then(data => {
        console.log("Successfully created room", data);
        createNotice.textContent = "Room created"
    })
    .catch(error => {
        console.error("An error occurred when creating a room", error);
    })
    createSubmit.disabled = false;
})