Import-Module "$PSScriptRoot/../siquery"

Describe 'etc_services table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$etc_services = $allSchemas | Where-Object { $_.BaseName -eq 'etc_services' }
				$etc_services | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$etc_services = $allSchemas | Where-Object { $_.BaseName -eq 'etc_services' }
				$schema = Get-Content $etc_services.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$etc_services = Get-Siq 'etc_services' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'etc_services' }).FullName -Raw

				$json = $etc_services | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
