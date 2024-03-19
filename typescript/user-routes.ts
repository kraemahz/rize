import express from 'express';
import bodyParser from 'body-parser';
import bcrypt from 'bcrypt';

const router = express.Router();
router.use(bodyParser.json());

// Placeholder database.
const usersDB = {};

// Signup endpoint
router.post('/signup', async (req, res) => {
    try {
        const { username, email, password } = req.body;
        // Perform input validation
        if (!username || !email || !password) {
            return res.status(400).json({ message: 'Please provide username, email and password.' });
        }
        // Check if user already exists
        if (usersDB[username]) {
            return res.status(409).json({ message: 'User already exists.' });
        }
        // Hash the password
        const salt = await bcrypt.genSalt(10);
        const passwordHash = await bcrypt.hash(password, salt);
        // Store the new user
        usersDB[username] = {
            email,
            passwordHash
        };
        res.status(201).json({ message: 'User created.' });
    } catch (error) {
        res.status(500).json({ message: 'Server error.' });
    }
});

// Read endpoint
router.get('/:username', (req, res) => {
    const { username } = req.params;
    const user = usersDB[username];
    if (user) {
        res.status(200).json({ username, email: user.email });
    } else {
        res.status(404).json({ message: 'User not found.' });
    }
});

// Update endpoint
router.put('/:username', async (req, res) => {
    const { username } = req.params;
    const { email, password } = req.body;
    const user = usersDB[username];

    if (user) {
        user.email = email || user.email;
        if (password) {
            const salt = await bcrypt.genSalt(10);
            user.passwordHash = await bcrypt.hash(password, salt);
        }
        res.status(200).json({ message: 'User updated.' });
    } else {
        res.status(404).json({ message: 'User not found.' });
    }
});

// Delete endpoint
router.delete('/:username', (req, res) => {
    const { username } = req.params;
    const user = usersDB[username];

    if (user) {
        delete usersDB[username];
        res.status(200).json({ message: 'User deleted.' });
    } else {
        res.status(404).json({ message: 'User not found.' });
    }
});

export default router;