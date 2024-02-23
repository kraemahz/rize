import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import UserProfileManagement from './UserProfileManagement';

describe('UserProfileManagement', () => {
  test('renders UserProfileManagement component', () => {
    render(<UserProfileManagement />);
    expect(screen.getByText(/save profile/i)).toBeInTheDocument();
  });

  test('updates profile data on user input', () => {
    render(<UserProfileManagement />);
    fireEvent.change(screen.getByLabelText(/name/i), { target: { value: 'John Doe' } });
    fireEvent.change(screen.getByLabelText(/biography/i), { target: { value: 'Biography text' } });
    expect(screen.getByLabelText(/name/i)).toHaveValue('John Doe');
    expect(screen.getByLabelText(/biography/i)).toHaveValue('Biography text');
  });

  // TODO: Add tests for profile picture upload
});
