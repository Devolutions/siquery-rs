Import-Module "$PSScriptRoot/../siquery"

Describe 'interface_address table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$interface_address = $allSchemas | Where-Object { $_.BaseName -eq 'interface_address' }
				$interface_address | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$interface_address = $allSchemas | Where-Object { $_.BaseName -eq 'interface_address' }
				$schema = Get-Content $interface_address.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$interface_address = Get-Siq 'interface_address' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'interface_address' }).FullName -Raw

				$json = $interface_address | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
