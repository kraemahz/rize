import express from 'express';
import bodyParser from 'body-parser';
import bcrypt from 'bcrypt';
import Joi from 'joi';

export const router = express.Router();
router.use(bodyParser.json());

interface User {
    email: string;
    passwordHash: string;
}

interface UsersDB {
    [username: string]: User | undefined;
}

export const usersDB: UsersDB = {};

// Define user validation schema
const userSchema = Joi.object({
    username: Joi.string().alphanum().min(3).max(30).required(),
    email: Joi.string().email({ tlds: { allow: false } }).required(),
    password: Joi.string().required()
});

router.post('/signup', async (req, res) => {
    const { error } = userSchema.validate(req.body);
    if (error) {
        return res.status(400).json({ message: 'Invalid user input', details: error.details });
    }
    const { username, email, password } = req.body;
    if (usersDB[username]) {
        return res.status(409).json({ message: 'User already exists.' });
    }
    const salt = await bcrypt.genSalt(10);
    const passwordHash = await bcrypt.hash(password, salt);
    usersDB[username] = { email, passwordHash };
    res.status(201).json({ message: 'User created.' });
});

router.get('/:username', (req, res) => {
    const { username } = req.params;
    const user = usersDB[username];
    if (!user) {
        return res.status(404).json({ message: 'User not found.' });
    }
    res.status(200).json({ username, email: user.email });
});

router.put('/:username', async (req, res) => {
    const { username } = req.params;
    const user = usersDB[username];
    if (!user) {
        return res.status(404).json({ message: 'User not found.' });
    }
    const { email, password } = req.body;
    user.email = email ?? user.email;
    if (password) {
        const salt = await bcrypt.genSalt(10);
        user.passwordHash = await bcrypt.hash(password, salt);
    }
    res.status(200).json({ message: 'User updated.' });
});

router.delete('/:username', (req, res) => {
    const { username } = req.params;
    const user = usersDB[username];
    if (!user) {
        return res.status(404).json({ message: 'User not found.' });
    }
    delete usersDB[username];
    res.status(200).json({ message: 'User deleted.' });
});

export default router;
