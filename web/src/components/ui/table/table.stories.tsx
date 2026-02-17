import type { Meta, StoryObj } from '@storybook/react';
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from './table';

const meta: Meta<typeof Table> = {
  title: 'UI/Table',
  component: Table,
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof Table>;

const sampleData = [
  { id: 1, name: 'Sensor A', location: 'Tokyo, Japan', status: 'Active', reading: '0.12 μSv/h' },
  { id: 2, name: 'Sensor B', location: 'Berlin, Germany', status: 'Active', reading: '0.08 μSv/h' },
  { id: 3, name: 'Sensor C', location: 'New York, USA', status: 'Warning', reading: '0.25 μSv/h' },
  { id: 4, name: 'Sensor D', location: 'London, UK', status: 'Active', reading: '0.09 μSv/h' },
  { id: 5, name: 'Sensor E', location: 'Paris, France', status: 'Offline', reading: 'N/A' },
];

export const Default: Story = {
  render: () => (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Name</TableHead>
          <TableHead>Location</TableHead>
          <TableHead>Status</TableHead>
          <TableHead>Reading</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {sampleData.map((row) => (
          <TableRow key={row.id}>
            <TableCell>{row.name}</TableCell>
            <TableCell>{row.location}</TableCell>
            <TableCell>{row.status}</TableCell>
            <TableCell>{row.reading}</TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  ),
};

export const Empty: Story = {
  render: () => (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Name</TableHead>
          <TableHead>Location</TableHead>
          <TableHead>Status</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow>
          <TableCell colSpan={3} className="text-center text-text-tertiary">
            No data available
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  ),
};

export const WithHover: Story = {
  render: () => (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Alert ID</TableHead>
          <TableHead>Severity</TableHead>
          <TableHead>Time</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow className="cursor-pointer">
          <TableCell>ALT-001</TableCell>
          <TableCell className="text-alert-critical">Critical</TableCell>
          <TableCell>2024-01-15 14:32:07</TableCell>
        </TableRow>
        <TableRow className="cursor-pointer">
          <TableCell>ALT-002</TableCell>
          <TableCell className="text-alert-high">High</TableCell>
          <TableCell>2024-01-15 13:45:22</TableCell>
        </TableRow>
        <TableRow className="cursor-pointer">
          <TableCell>ALT-003</TableCell>
          <TableCell className="text-alert-medium">Medium</TableCell>
          <TableCell>2024-01-15 12:18:45</TableCell>
        </TableRow>
      </TableBody>
    </Table>
  ),
};
