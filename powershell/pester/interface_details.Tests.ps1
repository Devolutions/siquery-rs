Import-Module "$PSScriptRoot/../siquery"

Describe 'interface_details table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$interface_details = $allSchemas | Where-Object { $_.BaseName -eq 'interface_details' }
				$interface_details | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$interface_details = $allSchemas | Where-Object { $_.BaseName -eq 'interface_details' }
				$schema = Get-Content $interface_details.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$interface_details = Get-Siq 'interface_details' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'interface_details' }).FullName -Raw

				$json = $interface_details | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
