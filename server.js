const express = require('express');
const app = express();
const PORT = process.env.PORT || 3000;

app.use(express.json());

let currentRotation = 0;
let lastSenderId = "";

app.post('/update-cframe', (req, res) => {
    if (req.body && typeof req.body.angle === 'number' && req.body.senderId) {
        currentRotation = req.body.angle;
        lastSenderId = req.body.senderId;
        return res.status(200).json({ success: true });
    }
    return res.status(400).json({ error: "Invalid data format" });
});

app.get('/get-cframe', (req, res) => {
    res.status(200).json({ angle: currentRotation, senderId: lastSenderId });
});

app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
