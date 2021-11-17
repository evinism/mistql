const usersWithAMessageEvent = {
  name: "Users with a Message Event",
  query: 'events | filter type == "send_message" | groupby email | keys',
  data: {
    "events": [
      {
        "type": "like",
        "email": "harold@example.com",
        "post_number": 5831
      },
      {
        "type": "send_message",
        "email": "flora@example.com",
        "message": "Hello, friend!",
        "targetUser": 95813
      },
      {
        "type": "like",
        "email": "flora@example.com",
        "post_number": 12385
      },
      {
        "type": "send_message",
        "email": "flora@example.com",
        "message": "I think you are cool!",
        "targetUser": 95813
      },
      {
        "type": "send_message",
        "email": "william@example.com",
        "message": "You Too!",
        "targetUser": 8381
      },
      {
        "type": "like",
        "email": "emma@example.com",
        "post_number": 17245
      },
      {
        "type": "like",
        "email": "flora@example.com",
        "post_number": 5831
      },
      {
        "type": "like",
        "email": "william@example.com",
        "post_number": 5831
      },
      {
        "type": "like",
        "email": "pete@example.com",
        "post_number": 17245
      }
    ]
  }
};

const usersWithMatchingEmails = {
  name: "User emails that match a regex",
  query: 'events | filter email =~ (regex "^[hf]")',
  data: {
    "events": [
      {
        "type": "like",
        "email": "harold@example.com",
        "post_number": 5831
      },
      {
        "type": "send_message",
        "email": "flora@example.com",
        "message": "Hello, friend!",
        "targetUser": 95813
      },
      {
        "type": "like",
        "email": "flora@example.com",
        "post_number": 12385
      },
      {
        "type": "send_message",
        "email": "flora@example.com",
        "message": "I think you are cool!",
        "targetUser": 95813
      },
      {
        "type": "send_message",
        "email": "william@example.com",
        "message": "You Too!",
        "targetUser": 8381
      },
      {
        "type": "like",
        "email": "emma@example.com",
        "post_number": 17245
      },
      {
        "type": "like",
        "email": "flora@example.com",
        "post_number": 5831
      },
      {
        "type": "like",
        "email": "william@example.com",
        "post_number": 5831
      },
      {
        "type": "like",
        "email": "pete@example.com",
        "post_number": 17245
      }
    ]
  }
};

const animalCounts = {
  name: "Animal Counts",
  query: "animals | groupby variety | mapvalues (count @)",
  data: {
    animals: [
      {
        variety: "cat",
        name: "Harold"
      },
      {
        variety: "dog",
        name: "Millie"
      },
      {
        variety: "cat",
        name: "Doggie"
      },
      {
        variety: "cat",
        name: "Jellybean"
      },
      {
        variety: "dog",
        name: "Bingo"
      },
    ],
  }
}

const oomBeforeConnect = {
  name: "Out of Memory before Connect",
  query: "alerts | groupby processid | mapvalues (sequence type==\"outofmemory\" type==\"connect\" @) | filtervalues (count @) > 0 | keys",
  data: {
    alerts: [
      {
        process: "server.exe",
        processid: "194",
        type: "outofmemory"
      },
      {
        process: "server.exe",
        processid: "195",
        type: "connect"
      },
      {
        process: "server.exe",
        processid: "195",
        type: "connect"
      },
      {
        process: "server.exe",
        processid: "195",
        type: "outofmemory"
      },
      {
        process: "server.exe",
        processid: "195",
        type: "outofmemory"
      },
      {
        process: "server.exe",
        processid: "194",
        type: "outofmemory"
      },
      {
        process: "server.exe",
        processid: "194",
        type: "connect"
      },
      {
        process: "server.exe",
        processid: "196",
        type: "connect"
      },
      {
        process: "server.exe",
        processid: "197",
        type: "outofmemory"
      },
      {
        process: "server.exe",
        processid: "197",
        type: "connect"
      }
    ]
  }
}

const examples = {
  usersWithAMessageEvent,
  usersWithMatchingEmails,
  animalCounts,
  oomBeforeConnect,
}

export default examples;
