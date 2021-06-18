Import-Module "$PSScriptRoot/../siquery"

Describe 'processes tests' {
	InModuleScope siquery {
		Context 'default' {
			It 'validates schema' {
				$table = Get-Siq 'processes' | Select-Object -First 1
				$schema = Get-Content "$PSScriptRoot\..\schema\processes.json" -Raw
				$schema | Test-Json | Should -Be $true
				$json = $table | ConvertTo-Json
				$json | Test-Json | Should -Be $true 
				$json | Test-Json -Schema $schema
			}
		}
	}
}
