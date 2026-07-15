const express = require('express');
const app = express();
const PORT = process.env.PORT || 3000;

app.use(express.json());

let currentCFrame = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1];

app.post('/update-cframe', (req, res) => {
    if (req.body && Array.isArray(req.body.cframe)) {
        currentCFrame = req.body.cframe;
        return res.status(200).json({ success: true });
    }
    return res.status(400).json({ error: "Invalid data format" });
});

app.get('/get-cframe', (req, res) => {
    res.status(200).json({ cframe: currentCFrame });
});

app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
