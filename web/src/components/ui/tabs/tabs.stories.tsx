import type { Meta, StoryObj } from '@storybook/react';
import { Tabs, TabsList, TabsTrigger, TabsContent } from './tabs';

const meta: Meta<typeof Tabs> = {
  title: 'UI/Tabs',
  component: Tabs,
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof Tabs>;

export const Default: Story = {
  render: () => (
    <Tabs defaultValue="account" className="w-[400px]">
      <TabsList>
        <TabsTrigger value="account">Account</TabsTrigger>
        <TabsTrigger value="password">Password</TabsTrigger>
        <TabsTrigger value="settings">Settings</TabsTrigger>
      </TabsList>
      <TabsContent value="account">
        <div className="p-4 text-[#a0a0b0]">Account settings content</div>
      </TabsContent>
      <TabsContent value="password">
        <div className="p-4 text-[#a0a0b0]">Password settings content</div>
      </TabsContent>
      <TabsContent value="settings">
        <div className="p-4 text-[#a0a0b0]">General settings content</div>
      </TabsContent>
    </Tabs>
  ),
};

export const WithDisabled: Story = {
  render: () => (
    <Tabs defaultValue="tab1" className="w-[400px]">
      <TabsList>
        <TabsTrigger value="tab1">Active</TabsTrigger>
        <TabsTrigger value="tab2" disabled>
          Disabled
        </TabsTrigger>
        <TabsTrigger value="tab3">Another</TabsTrigger>
      </TabsList>
      <TabsContent value="tab1">
        <div className="p-4 text-[#a0a0b0]">Tab 1 content</div>
      </TabsContent>
      <TabsContent value="tab2">
        <div className="p-4 text-[#a0a0b0]">Tab 2 content</div>
      </TabsContent>
      <TabsContent value="tab3">
        <div className="p-4 text-[#a0a0b0]">Tab 3 content</div>
      </TabsContent>
    </Tabs>
  ),
};
