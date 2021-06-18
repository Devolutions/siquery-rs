Import-Module "$PSScriptRoot/../siquery"

Describe 'os_version tests' {
	InModuleScope siquery {
		Context 'default' {
			It 'validates schema' {
				$os_version = Get-Siq 'os_version' | Select-Object -First 1
				$schema = Get-Content "$PSScriptRoot\..\schema\os_version.json" -Raw
				$schema | Test-Json | Should -Be $true
				$json = $os_version | ConvertTo-Json
				$json | Test-Json | Should -Be $true 
				$json | Test-Json -Schema $schema
			}
		}
	}
}
