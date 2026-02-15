import { createSignal } from 'solid-js';

function Header() {
  const [searchQuery, setSearchQuery] = createSignal('');
  
  return (
    <header class="h-16 bg-[#12121a] border-b border-[#2a2a3a] flex items-center justify-between px-6">
      <div class="flex items-center gap-4 flex-1">
        <div class="relative flex-1 max-w-md">
          <input
            type="text"
            placeholder="Search sensors, locations, incidents..."
            value={searchQuery()}
            onInput={(e) => setSearchQuery(e.currentTarget.value)}
            class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 pl-10 text-sm focus:outline-none focus:border-[#00d4ff] transition-colors"
          />
          <span class="absolute left-3 top-2.5 text-gray-500">🔍</span>
        </div>
      </div>
      
      <div class="flex items-center gap-6">
        <div class="flex items-center gap-2">
          <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
          <span class="text-sm text-gray-400">System Normal</span>
        </div>
        
        <div class="h-6 w-px bg-[#2a2a3a]"></div>
        
        <button class="relative p-2 text-gray-400 hover:text-gray-200 transition-colors">
          <span class="text-lg">🔔</span>
          <span class="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full"></span>
        </button>
        
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-full bg-[#00d4ff]/20 border border-[#00d4ff]/30 flex items-center justify-center text-[#00d4ff] font-medium text-sm">
            TW
          </div>
        </div>
      </div>
    </header>
  );
}

export default Header;
