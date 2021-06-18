Import-Module "$PSScriptRoot/../siquery"

Describe 'uptime tests' {
	InModuleScope siquery {
		Context 'default' {
			It 'validates schema' {
				$uptime = Get-Siq 'uptime' | Select-Object -First 1
				$schema = Get-Content "$PSScriptRoot\..\schema\uptime.json" -Raw
				$schema | Test-Json | Should -Be $true
				$json = $uptime | ConvertTo-Json
				$json | Test-Json | Should -Be $true 
				$json | Test-Json -Schema $schema
			}
		}
	}
}
