const btn = document.querySelector('#sec-text button');
const textarea = document.getElementById('sec-textarea');

btn.addEventListener('click', () => {
    if (textarea) {
	sendData({val: textarea.value}); // send a test JSON object
	clearTextArea();
    }
});

function clearTextArea() {
    if (textarea) {
	textarea.value = "";
    }
}

function sendData(data) {
    console.log('data:', data);

    fetch('/minitwitter.html', {
	method: 'POST',
	headers: {
	    'Content-Type': 'application/json',
	},
	body: JSON.stringify(data), // convert data into a JSON string
    })
	.then(res => res.json()) // read the response stream to completion; resolve to a JSON object
	.then(data => {
	    console.log('Success:', data);
	})
	.catch(err => {
	    console.log('Error:', err);
	    alert('Error getting response');
	});
}
