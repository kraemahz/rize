import request from 'supertest';
import express from 'express';
import { router as userRoutes } from './user-routes';

const app = express();
app.use(express.json());
app.use('/user', userRoutes);

describe('User Routes Tests', () => {
    test('POST /user/signup - success', async () => {
        const response = await request(app)
            .post('/user/signup')
            .send({ username: 'testuser', email: 'test@example.com', password: 'password123' });
        expect(response.statusCode).toBe(201);
        expect(response.body.message).toEqual('User created.');
    });

    test('GET /user/testuser - success', async () => {
        await request(app)
            .post('/user/signup')
            .send({ username: 'testuser', email: 'test@example.com', password: 'password123' });

        const response = await request(app)
            .get('/user/testuser');
        expect(response.statusCode).toBe(200);
        expect(response.body).toEqual({ username: 'testuser', email: 'test@example.com' });
    });

    test('PUT /user/testuser - success', async () => {
        await request(app)
            .post('/user/signup')
            .send({ username: 'testuser', email: 'test@example.com', password: 'password123' });

        const response = await request(app)
            .put('/user/testuser')
            .send({ email: 'newemail@example.com', password: 'newpassword123' });
        expect(response.statusCode).toBe(200);
        expect(response.body.message).toEqual('User updated.');
    });

    test('DELETE /user/testuser - success', async () => {
        await request(app)
            .post('/user/signup')
            .send({ username: 'testuser', email: 'test@example.com', password: 'password123' });

        const response = await request(app)
            .delete('/user/testuser');
        expect(response.statusCode).toBe(200);
        expect(response.body.message).toEqual('User deleted.');
    });
});